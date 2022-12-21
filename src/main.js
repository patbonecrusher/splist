const { invoke } = window.__TAURI__.tauri;

let portCount;

async function show_count() {
    const count = await invoke('port_count');
    portCount.textContent = count;

    for (i=0; i<count; ++i) {
        console.log(await invoke('port_at_index', { idx: i }));
    }
}

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
//   greetMsgPort.textContent = await invoke("listport");
// }

window.addEventListener("DOMContentLoaded", () => {
  portCount = document.querySelector("#port_count");
  show_count();
//   document
//     .querySelector("#greet-button")
//     .addEventListener("click", () => greet());
});
