import { invoke } from "@tauri-apps/api/tauri";
type FidoDeviceList = import("./schemas").FidoDeviceList;
type FidoGetInfoCommand = import("./schemas").FidoGetInfoCommand;
type FidoGetInfoResponse = import("./schemas").FidoGetInfoResponse;
type FidoSetPinCommand = import("./schemas").FidoSetPinCommand;
type FidoSetPinResponse = import("./schemas").FidoSetPinResponse;
type FidoChangePinCommand = import("./schemas").FidoChangePinCommand;
type FidoChangePinResponse = import("./schemas").FidoChangePinResponse;
type FidoResetCommand = import("./schemas").FidoResetCommand;
type FidoResetResponse = import("./schemas").FidoResetResponse;

const fido_init = () => {
  console.log("fido_init");
  invoke("fido_init");
};

const fido_list_devices = (): Promise<FidoDeviceList> => {
  console.log("fido_list_devices");
  return invoke("fido_list_devices");
};

async function fido_get_info(
  parameters: FidoGetInfoCommand
): Promise<FidoGetInfoResponse> {
  console.log("fido_get_info");
  return invoke("fido_get_info", { parameters: parameters });
}

async function fido_set_pin(parameters: FidoSetPinCommand): Promise<FidoSetPinResponse> {
  console.log("fido_set_pin");
  return invoke("fido_set_pin", { parameters: parameters });
}

async function fido_change_pin(parameters: FidoChangePinCommand): Promise<FidoChangePinResponse> {
  console.log("fido_change_pin");
  return invoke("fido_change_pin", { parameters: parameters });
}

async function fido_reset(parameters: FidoResetCommand): Promise<FidoResetResponse> {
  console.log("fido_reset");
  return invoke("fido_reset", { parameters: parameters });
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

let fido_first_device_path: NonNullable<string> = "";

window.addEventListener("DOMContentLoaded", () => {
  fido_init();

  fido_list_devices().then((device_list) => {
    fido_first_device_path = device_list.dev[0];

    console.log(fido_first_device_path);

    if (fido_first_device_path) {
      fido_get_info({ dev: fido_first_device_path } as FidoGetInfoCommand).then(_ =>
        console.log(JSON.stringify(_, null, 4))
      );
    }
  });

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
  fido_reset({ dev: "${fido_first_device_path}" } as FidoResetCommand).then(
    (_) => {
      console.log(_);
      window.location.hash = "";
    }
  );
};
