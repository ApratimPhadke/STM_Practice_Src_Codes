<div align="center">
  <h1>🚀 The STM32 Bare-Metal Bible</h1>
  <p><i>A comprehensive collection of C programming and bare-metal practice codes for STM32 microcontrollers.</i></p>
</div>

---

## 📖 About This Repository
Welcome to the **STM32 Bare-Metal Bible**! This repository is designed to take you from the fundamentals of C programming all the way to advanced bare-metal embedded concepts. Whether you are brushing up on pointers, mastering bitwise operations for register manipulation, or exploring memory alignment, this repo has a dedicated practice code for it. 

The codes are structured to be hardware-agnostic at the core logic level, making it easy to run them on almost any STM32 development board.

---

## 🗂️ Repository Structure (Source Codes)

The repository contains numerous modular practice folders, categorized below for your learning journey:

### 🧮 1. C Fundamentals & Math
* `ASCII` - Understanding character encoding.
* `Average` - Basic arithmetic and logic.
* `math` - Core mathematical operations.
* `constdata` - Understanding the `const` qualifier in ROM/RAM.
* `macro` - Preprocessor directives and macros.
* `type_casting` - Implicit and explicit type conversions.

### 🔀 2. Control Flow & Logic
* `ifelseBiggestOfTwo`, `ifelseCastVote`, `ifelseIfTax`, `ifstatementCastVote` - Conditional branching logic.
* `switchCaseArea` - State machine basics using switch-case.
* `WhileLoop1to10`, `whileLoopEvenNum` - Indefinite looping constructs.
* `forLoopEvenNum`, `forloopNumberPyramid` - Definite looping and pattern generation.

### 📦 3. Arrays & Strings
* `arrays` - Array initialization and memory layout.
* `arrays_passing` - Passing arrays to functions (decaying to pointers).
* `arrays_swap` - Manipulating array data.
* `strings` - Character arrays and string manipulation in embedded systems.

### 🎯 4. Pointers & Memory
* `pointer`, `pointer_1`, `pointer_2` - Pointer arithmetic, dereferencing, and memory addresses.
* `var_address` - Inspecting variables in the memory map.
* `function` - Function pointers and modular code.

### 🏗️ 5. Structures, Unions & Alignment
* `structInit`, `struct_array`, `structPointer` - Organizing complex data types.
* `structPacket`, `structPacketBitfield` - Creating protocol packets and bit-fields.
* `student_record` - Real-world data organization.
* `union`, `unionBitExtraction` - Memory sharing and extracting specific bytes/bits from registers.
* `dataAlignment` - Understanding struct padding and memory alignment constraints on ARM Cortex-M.

### ⚙️ 6. Bitwise Operations (Crucial for Bare-Metal)
* `bitWise2Numbers` - Basic AND, OR, XOR, NOT operations.
* `bitWiseEvenOdd` - Using bitwise logic for optimization.
* `bitWiseSet` - Setting, clearing, and toggling specific bits (Register manipulation prep).

---

## 🛠️ Hardware Requirements
The beauty of bare-metal C is its portability. You can run these codes on almost **any STM32 board**. 

**Recommended Boards:**
* 💊 **STM32 Bluepill** (STM32F103C8T6) - The classic beginner board.
* 💊 **STM32 Blackpill** (STM32F401CCU6 / STM32F411CEU6) - Higher performance with floating-point units.
* 🛡️ **STM32 Nucleo Boards** (e.g., Nucleo-F446RE, Nucleo-G071RB) - Official ST boards with built-in programmers.
* *Any other custom STM32 board.*

**Required Programmer:**
* **ST-Link V2** (USB Dongle) - If using a Bluepill/Blackpill. 
    * *Connections:* `SWDIO` -> `SWDIO`, `SWCLK` -> `SWCLK`, `GND` -> `GND`, `3.3V` -> `3.3V`.
* *(Nucleo boards already have an ST-Link built-in!)*

---

## 💻 Software Requirements
* [**STM32CubeIDE**](https://www.st.com/en/development-tools/stm32cubeide.html) - ST's official, free Eclipse-based IDE.

---

## 🚀 How to Extract and Use This Repository

Follow these steps to get the code running on your microcontroller:

### Step 1: Download & Extract
1. Click the **`Code`** button at the top right of this repository and select **`Download ZIP`**.
2. Extract the `.zip` file to a known folder on your computer (e.g., `C:\STM32_Workspace`).

### Step 2: Setup STM32CubeIDE
1. Open **STM32CubeIDE**.
2. Create a new project: `File` -> `New` -> `STM32 Project`.
3. Select your specific Microcontroller/Board (e.g., `STM32F411CE` for Blackpill).
4. Give your project a name (e.g., `BareMetal_Practice`).
5. Choose **Empty** project type (or Basic) since we are focusing on bare-metal and core C, bypassing HAL configurations if preferred.

### Step 3: Import the Code
1. Navigate to the extracted repository folder.
2. Open the specific folder you want to practice (e.g., `pointer_1`).
3. Copy the `.c` and `.h` files.
4. Go back to STM32CubeIDE, right-click on the `Core/Src` folder of your newly created project, and click **Paste**. (Replace `main.c` if asked).

### Step 4: Build and Flash
1. Click the 🔨 **Build (Hammer)** icon in the toolbar to compile the code. Ensure there are 0 errors.
2. Connect your STM32 board via the ST-Link V2 to your computer.
3. Click the 🐞 **Debug (Bug)** icon to flash the code onto the microcontroller.
4. You can now step through the code line-by-line, inspect variable addresses, registers, and memory right inside the IDE!

---

## 💡 Why Bare-Metal?
Understanding bare-metal programming and the underlying C concepts (like bitwise math, pointers, and memory alignment) is what separates a library-user from a true Embedded Systems Engineer. By mastering these snippets, you are preparing yourself to write your own drivers, understand hardware reference manuals, and optimize memory on ARM Cortex-M processors.

Happy Coding! ⚡
