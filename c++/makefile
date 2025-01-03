# Variables comunes
INCLUDE_DIR = include
SRC = main.cpp engine.cpp scenes/lorenz_attractor.cpp utils/rk4.cpp
TARGET_WASM_DIR = target/wasm
TARGET_NATIVE_DIR = target
TARGET_WASM = $(TARGET_WASM_DIR)/lorenz.html
TARGET_NATIVE = $(TARGET_NATIVE_DIR)/lorenz

# Flags para WebAssembly
EMCC_FLAGS = -I$(INCLUDE_DIR) -s USE_SDL=2 -s WASM=1 -s DISABLE_EXCEPTION_CATCHING=0

# Flags para compilación nativa
GXX_FLAGS = -I$(INCLUDE_DIR)

# Reglas
.PHONY: all wasm native clean

# Compilar todo
all: wasm native

# Compilar para WebAssembly
wasm: $(TARGET_WASM)

$(TARGET_WASM): $(SRC)
	mkdir -p $(TARGET_WASM_DIR)
	emcc $(EMCC_FLAGS) -o $@ $^

# Compilar para nativo
native: $(TARGET_NATIVE)

$(TARGET_NATIVE): $(SRC)
	mkdir -p $(TARGET_NATIVE_DIR)
	g++ $(SRC) $(GXX_FLAGS) -o $@ -lSDL2

# Limpiar binarios
clean:
	rm -rf $(TARGET_WASM_DIR) $(TARGET_NATIVE_DIR)
