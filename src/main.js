const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let greetMsgPort;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
  greetMsgPort.textContent = await invoke("listport");
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  greetMsgPort = document.querySelector("#greet-port");
  document
    .querySelector("#greet-button")
    .addEventListener("click", () => greet());
});
