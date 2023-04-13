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

async function fido_set_pin(
  parameters: FidoSetPinCommand
): Promise<FidoSetPinResponse> {
  console.log("fido_set_pin");
  return invoke("fido_set_pin", { parameters: parameters });
}

async function fido_change_pin(
  parameters: FidoChangePinCommand
): Promise<FidoChangePinResponse> {
  console.log("fido_change_pin");
  return invoke("fido_change_pin", { parameters: parameters });
}

async function fido_reset(
  parameters: FidoResetCommand
): Promise<FidoResetResponse> {
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

const hideElementById = (elementId: string): void => {
  (document.getElementById(elementId) as HTMLElement).style.display = "none";
};

const showElementById = (elementId: string): void => {
  (document.getElementById(elementId) as HTMLElement).style.display = "inline";
};

const onHashChange = (hash: string) => {
  console.log("navigating to " + hash);

  hideElementById(_currentHandler.element);

  _currentHandler.onUnload();

  _currentHandler = _handlers[hash] || _defaultHandler;

  _currentHandler.onLoad();

  showElementById(_currentHandler.element);
};

addHandler({
  route: "",
  element: "main",
  onLoad: () => {
    console.log(fido_first_device_path);

    if (fido_first_device_path) {
      fido_get_info({ dev: fido_first_device_path } as FidoGetInfoCommand).then(
        (_) => {
          console.log(JSON.stringify(_, null, 4));

          if (_?.options?.clientPin == true) {
            hideElementById("main-set-pin-enabled");
            showElementById("main-set-pin-disabled");
            showElementById("main-change-pin-enabled");
            hideElementById("main-change-pin-disabled");
          } else if (_?.options?.clientPin == false) {
            showElementById("main-set-pin-enabled");
            hideElementById("main-set-pin-disabled");
            hideElementById("main-change-pin-enabled");
            showElementById("main-change-pin-disabled");
          } else {
            hideElementById("main-set-pin-enabled");
            hideElementById("main-set-pin-disabled");
            hideElementById("main-change-pin-enabled");
            hideElementById("main-change-pin-disabled");
          }
        }
      );
    }

    return;
  },
  onUnload: () => {
    return;
  },
} as PageHandler);

addHandler({
  route: "#setpin",
  element: "setpin",
  onLoad: () => {
    (document.getElementById("setPinInput") as HTMLInputElement).value = "";
    (document.getElementById("setPinConfirm") as HTMLInputElement).value = "";
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

    window.addEventListener("hashchange", () => {
      onHashChange(window.location.hash);
    });

    // force reload of start page
    window.location.hash = "";
  });
});

declare global {
  interface Window {
    showSetPinInput: () => void;

    showSetPinConfirm: () => void;

    showCurrentPin: () => void;

    showNewPin: () => void;

    showNewPinConfirm: () => void;

    setPin: () => void;

    changePin: () => void;

    reset: () => void;
  }
}

const showPin = (elementId: string) => () => {
  const x = <HTMLInputElement>document.getElementById(elementId);

  x.type = x.type === "password" ? "text" : "password";
};

window.showSetPinInput = showPin("setPinInput");

window.showSetPinConfirm = showPin("setPinConfirm");

window.showCurrentPin = showPin("currentPinInput");

window.showNewPin = showPin("newPinInput");

window.showNewPinConfirm = showPin("newPinConfirm");

window.setPin = () => {
  console.log(fido_first_device_path);
  const newPin = (document.getElementById("setPinInput") as HTMLInputElement)
    .value;
  fido_set_pin({
    dev: fido_first_device_path,
    newPin: newPin,
  } as FidoSetPinCommand)
    .then((_) => {
      window.location.hash = "";
    })
    .catch((_) => {
      console.log(_);
    });
};

window.changePin = () => {
  console.log(fido_first_device_path);
  const newPin = (document.getElementById("newPinInput") as HTMLInputElement)
    .value;
  const oldPin = (
    document.getElementById("currentPinInput") as HTMLInputElement
  ).value;
  fido_change_pin({
    dev: fido_first_device_path,
    newPin: newPin,
    oldPin: oldPin,
  } as FidoChangePinCommand)
    .then((_) => {
      window.location.hash = "";
    })
    .catch((_) => {
      console.log(_);
    });
};

window.reset = () => {
  console.log(fido_first_device_path);
  fido_reset({ dev: fido_first_device_path } as FidoResetCommand)
    .then((_) => {
      window.location.hash = "";
    })
    .catch((_) => {
      console.log(_);
    });
};
