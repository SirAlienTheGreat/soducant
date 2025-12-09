import init, {
  solve,
  get_random_puzzle,
  benchmark_intern,
} from "./pkg/soducant_wasm.js";

var solutionVisible = false;
var allEntries = [];

init();

function generateSudokuBoard() {
  const table = document.getElementById("sudokuBoard");

  // Clear any existing rows
  table.innerHTML = "";

  // Create 9 rows
  for (let row = 1; row <= 9; row++) {
    const tr = document.createElement("tr");

    // Create 9 cells per row
    for (let col = 1; col <= 9; col++) {
      const th = document.createElement("th");

      const input = document.createElement("input");
      input.type = "number";
      input.classList.add("sudokuEntry");
      input.min = 1;
      input.max = 9;

      // Give each cell a unique ID like s1, s2, ..., s81
      const cellNum = (row - 1) * 9 + col;
      const cellId = "s" + cellNum;
      input.id = cellId;

      // Handle clearing inputs on entry
      input.addEventListener("input", clearPlaceholders);
      allEntries.push(input);

      // Handle moving to new boxes on input
      input.addEventListener("keydown", function (e) {
        var nextInputId = 0;
        switch (e.key) {
          case "ArrowUp":
            nextInputId = (cellNum + 72) % 81;
            if (nextInputId == 0) {
              nextInputId = 81;
            }
            break;
          case "ArrowDown":
            nextInputId = (cellNum + 9) % 81;
            if (nextInputId == 0) {
              nextInputId = 81;
            }
            break;
          case "ArrowRight":
            nextInputId = cellNum + 1;
            if (nextInputId == 82) {
              nextInputId = 1;
            }
            break;
          case "ArrowLeft":
            nextInputId = cellNum - 1;
            if (nextInputId == 0) {
              nextInputId = 81;
            }
            break;
          default:
            return;
        }
        const nextInput = document.getElementById("s" + nextInputId);
        if (nextInput) {
          e.preventDefault();
          nextInput.focus();
        } else {
          console.log(nextInputId);
        }
      });

      /*input.addEventListener("onkeypress", function (e) {
        return false;
        });*/

      th.appendChild(input);
      tr.appendChild(th);
      if (col % 3 == 0) {
        const spacer = document.createElement("col");
        spacer.classList.add("spacer");
        tr.appendChild(spacer);
      }
    }

    table.appendChild(tr);

    if (row % 3 == 0) {
      const spacer = document.createElement("row");
      spacer.classList.add("spacer");
      table.appendChild(spacer);
    }
  }
}

function clearPlaceholders() {
  if (!solutionVisible) {
    return;
  }
  allEntries.forEach((el) => {
    el.placeholder = "";
  });
  solutionVisible = false;
}

function readSudoku() {
  var string = "";
  for (var i = 1; i < 82; i++) {
    var newElement = document.getElementById("s" + i).value;
    if (newElement == "") {
      string = string + "0";
      continue;
    }
    string = string + newElement;
  }
  return string;
}

function writeSudoku(puzzle) {
  for (var i = 1; i < 82; i++) {
    if (puzzle[i - 1] == 0) {
      document.getElementById("s" + i).value = "";
      continue;
    }
    document.getElementById("s" + i).value = puzzle[i - 1];
  }
  clearPlaceholders();
}

function writeSudokuSolution(puzzle) {
  for (var i = 1; i < 82; i++) {
    if (puzzle[i - 1] == 0) {
      continue;
    }
    document.getElementById("s" + i).placeholder = puzzle[i - 1];
  }
  solutionVisible = true;
}

function solveSudoku() {
  const puzzle = readSudoku();

  const start = performance.now();
  const solution = solve(puzzle);
  const end = performance.now();

  if (solution == "Error") {
    document.getElementById("output").innerHTML = "Couldn't solve Sudoku";
    return;
  }

  document.getElementById("output").innerHTML =
    "Solved sudoku in " + (end - start) + "ms";

  writeSudokuSolution(solution);
}

function generateNewBoard() {
  clearPlaceholders();
  writeSudoku(get_random_puzzle());
}

async function run_web_benchmark() {
  const tries = 1000;

  const start = performance.now();
  for (var i = 0; i < tries; i++) {
    generateNewBoard();
    solveSudoku();
    await new Promise(requestAnimationFrame);
  }
  const end = performance.now();

  document.getElementById("output").innerHTML =
    "Benchmark (web edition, including writing to page) solved " +
    tries +
    " sudokus in " +
    (end - start) +
    "ms";
}

function run_internal_benchmark() {
  const start = performance.now();
  benchmark_intern();
  benchmark_intern();
  const end = performance.now();

  document.getElementById("output").innerHTML =
    "Benchmark (internal edition, no writing to page) solved " +
    1000 +
    " sudokus in " +
    (end - start) +
    "ms";
}

generateSudokuBoard();

window.readSudoku = readSudoku;
window.writeSudoku = writeSudoku;
window.writeSudokuSolution = writeSudokuSolution;
window.solveSudoku = solveSudoku;
window.generateNewBoard = generateNewBoard;
window.run_web_benchmark = run_web_benchmark;
window.run_internal_benchmark = run_internal_benchmark;
window.solve = solve;
