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

interface PageHandler {
  route: string;
  element: string;
  onLoad(): void;
  onUnload(): void;
}

let _currentPage: PageHandler;
const _pageHandlers = {};

const addHandler = (handler: PageHandler) => {
  if (handler.route == "") _currentPage = handler;

  _pageHandlers[handler.route] = handler;
};

const onHashChange = (hash: string) => {
  (document.getElementById(_currentPage.element) as HTMLElement).style.display =
    "none";

  _currentPage.onUnload();

  _currentPage = _pageHandlers[hash] || _pageHandlers[""];

  _currentPage.onLoad();

  (document.getElementById(_currentPage.element) as HTMLElement).style.display =
    "inline";
};

addHandler({
  route: "",
  element: "main",
  onLoad: () => {
    return;
  },
  onUnload: () => {
    return;
  },
} as PageHandler);

addHandler({
  route: "#changepin",
  element: "changepin",
  onLoad: () => {
    (document.getElementById("currentPinInput") as HTMLInputElement).value = "";
    (document.getElementById("newPinInput") as HTMLInputElement).value = "";
    (document.getElementById("newPinConfirm") as HTMLInputElement).value = "";
  },
  onUnload: () => {
    return;
  },
} as PageHandler);

window.addEventListener("DOMContentLoaded", () => {
  window.addEventListener("hashchange", () => {
    onHashChange(window.location.hash);
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
