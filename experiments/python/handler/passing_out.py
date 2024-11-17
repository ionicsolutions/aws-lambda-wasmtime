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

print("[bold]Call a function exported by the guest:[/bold]")
wasm_add = instance.exports(store)["add"]

print(f"1 + 2 = {wasm_add(store, 1, 2)}")
print(f"2 + 3 = {wasm_add(store, 2, 3)}")
print(f"3 + 4 = {wasm_add(store, 3, 4)}")

print("[bold]Have the guest call a host function to report the result:[/bold]")
wasm_calculate_and_report = instance.exports(store)["calculate_and_report"]


def calculate_and_report(number: int) -> list[int]:
    factors.clear()
    wasm_calculate_and_report(store, number)
    return factors.copy()


print(f"123456789 -> {calculate_and_report(123456789)}")
print(f"12345789 -> {calculate_and_report(12345789)}")
print(f"1236789 -> {calculate_and_report(1236789)}")


print("[bold]Have the guest that return a pointer to a string in memory:[/bold]")
# wasm_calculate_and_store_json = instance.exports(store)['calculate_and_store_json']
wasm_calculate_and_store_json = instance.exports(store)[
    "calculate_and_store_json_no_forget"
]

memory = instance.exports(store)["memory"]

for number in [45477787, 645640456, 12345]:
    ptr = wasm_calculate_and_store_json(store, number)

    # We read the entire memory from the pointer to the end.
    # Ideally, we would also know how many bytes to read, but
    # since we apparently cannot write Rust functions that return
    # more than one value at the moment (see README) we're stuck
    # with this.
    # There are plenty of ways to work around this, e.g., read in
    # chunks until we find the end, store the length of the string
    # in memory...
    long_bytes = memory.read(store, ptr, memory.data_len(store))

    raw_response = ""
    curly_counter = 0
    for i, char in enumerate(map(chr, long_bytes)):
        if curly_counter == 0 and char != "{":
            raise ValueError("Expected '{' at the beginning of the JSON string.")

        if char == "{":
            curly_counter += 1
        elif char == "}":
            curly_counter -= 1

        raw_response += char

        if curly_counter == 0:
            print(
                f"  [italic]End of JSON string reached: {long_bytes[i-3:i+3:]}[/italic]"
            )
            break

    print(f"{number} -> {json.loads(raw_response)}")
