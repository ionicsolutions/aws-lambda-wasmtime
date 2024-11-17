import json
import pathlib

from rich import print
from wasmtime import Store, Module, WasiConfig, Engine, Linker, FuncType, ValType

PATH = pathlib.Path(__file__).parent
module = PATH.parent / "function/target/wasm32-wasip1/release/function.wasm"
assert module.exists()

engine = Engine()
linker = Linker(engine)
# Set up the WASI standard imports
linker.define_wasi()

config = WasiConfig()
store = Store(engine)
store.set_wasi(config)

# Create a custom function that allows the Wasm module to
# write into the host environment
factors = []


def receive_factor(a):
    factors.append(a)


linker.define_func(
    "host", "report_factor", FuncType([ValType.i64()], []), receive_factor
)

module = Module.from_file(store.engine, module.absolute())
instance = linker.instantiate(store, module)

calculate_prime_factors = instance.exports(store)["calculate_and_print_json"]

print("[bold]Have the guest inherit stdout:[/bold]")

print(f"123456789 ->")

config = WasiConfig()
config.inherit_stdout()
store.set_wasi(config)

calculate_prime_factors(store, 123456789)

print("[bold]Send the guest's stdout to a file:[/bold]")

config = WasiConfig()
config.stdout_file = "output.txt"
store.set_wasi(config)

for number in [478899, 7989797843, 321324]:
    print(f"{number} ->")
    calculate_prime_factors(store, number)

    with open("output.txt") as f:
        *_, result = f.readlines()
        print(json.loads(result))

with open("output.txt") as f:
    print("  [italic]output.txt[/italic]:")
    print(f.read())

print("[bold]Pass number as an environment variable:[/bold]")

read_from_env_and_report = instance.exports(store)["read_from_env_and_report"]

assert not factors

config = WasiConfig()
config.env = [("NUMBER", "99")]
store.set_wasi(config)

read_from_env_and_report(store)

assert factors == [3, 3, 11]
print(f"NUMBER=99 -> {factors}")
factors.clear()

print("[bold]Pass numbers as arguments:[/bold]")

read_from_args_and_report = instance.exports(store)["read_from_args_and_report"]

assert not factors

config = WasiConfig()
config.argv = ["some-name-of-the-program", "456"]
store.set_wasi(config)

read_from_args_and_report(store)

assert factors == [2, 2, 2, 3, 19]
print(f"456 -> {factors}")
factors.clear()
