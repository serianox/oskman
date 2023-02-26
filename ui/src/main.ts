import { invoke } from "@tauri-apps/api/tauri";
type CustomResponse = import("./schemas").CustomResponse;
type FidoResetCommand = import("./schemas").FidoResetCommand;
type FidoResetResponse = import("./schemas").FidoResetResponse;

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

const fido_init = (flags: number) => {
  invoke("fido_init", { flags: flags });
};

async function fido_reset(dev: FidoResetCommand): Promise<FidoResetResponse> {
  console.log("fido_reset");
  return invoke("fido_reset", { parameters: dev });
}

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

let _defaultHandler: PageHandler;
let _currentHandler: PageHandler;
const _handlers = {};

const addHandler = (handler: PageHandler) => {
  if (_currentHandler == null) {
    _currentHandler = handler;
  }

  if (handler.route == "") {
    _defaultHandler = handler;
  }

  _handlers[handler.route] = handler;
};

const onHashChange = (hash: string) => {
  // console.log(hash);

  (
    document.getElementById(_currentHandler.element) as HTMLElement
  ).style.display = "none";

  _currentHandler.onUnload();

  _currentHandler = _handlers[hash] || _defaultHandler;

  _currentHandler.onLoad();

  (
    document.getElementById(_currentHandler.element) as HTMLElement
  ).style.display = "inline";
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

addHandler({
  route: "#reset",
  element: "reset",
  onLoad: () => {
    (document.getElementById("resetConfirm") as HTMLInputElement).value = "";
  },
  onUnload: () => {
    return;
  },
} as PageHandler);

window.addEventListener("DOMContentLoaded", () => {
  fido_init(0);

  window.addEventListener("hashchange", () => {
    onHashChange(window.location.hash);
  });
});

declare global {
  interface Window {
    showCurrentPin: () => void;

    showNewPin: () => void;

    showNewPinConfirm: () => void;

    reset: () => void;
  }
}

const showPin = (elementId: string) => () => {
  const x = <HTMLInputElement>document.getElementById(elementId);

  x.type = x.type === "password" ? "text" : "password";
};

window.showCurrentPin = showPin("currentPinInput");

window.showNewPin = showPin("newPinInput");

window.showNewPinConfirm = showPin("newPinConfirm");

window.reset = () => {
  fido_reset({ dev: "Foo" } as FidoResetCommand).then((_) => {
    console.log(_);
    window.location.hash = "";
  });
};
