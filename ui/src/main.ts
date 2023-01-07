import { invoke } from "@tauri-apps/api/tauri";
type CustomResponse = import("./schemas").CustomResponse;

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    }).then((_) => (_ as CustomResponse).message);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  window.addEventListener("hashchange", () => {
    const hash = window.location.hash;

    console.log(hash);
  });
});

declare global {
  interface Window {
    showCurrentPin: () => void;

    showNewPin: () => void;

    showNewPinConfirm: () => void;
  }
}

const showPin = (elementId: string) => () => {
  const x = <HTMLInputElement>document.getElementById(elementId);

  x.type = x.type === "password" ? "text" : "password";
};

window.showCurrentPin = showPin("currentPinInput");

window.showNewPin = showPin("newPinInput");

window.showNewPinConfirm = showPin("newPinConfirm");
