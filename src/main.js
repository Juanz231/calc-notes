const { invoke } = window.__TAURI__.core;

let gradeInputEl;
let percentageInputEl;
let resultMsgEl;

async function calculateMinGrade() {
  const grade = parseFloat(gradeInputEl.value);
  const percentage = parseFloat(percentageInputEl.value);

  resultMsgEl.textContent = await invoke("calculate_min_grade", { grade, percentage });
}

window.addEventListener("DOMContentLoaded", () => {
  gradeInputEl = document.querySelector("#grade-input");
  percentageInputEl = document.querySelector("#percentage-input");
  resultMsgEl = document.querySelector("#result-msg");
  document.querySelector("#grade-form").addEventListener("submit", (e) => {
    e.preventDefault();
    calculateMinGrade();
  });
});
