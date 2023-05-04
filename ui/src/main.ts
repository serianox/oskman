// Import our custom CSS
//import "scss/styles.scss";
import "bootstrap/dist/css/bootstrap.css";

// Import all of Bootstrap's JS
import /** as bootstrap from */ "bootstrap";

import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";

import { Fido } from "./fido";
import { FidoDeviceList } from "./schemas";

appWindow.theme().then(
  (theme) => {
    (document.documentElement as HTMLElement).setAttribute(
      "data-bs-theme",
      theme?.toString() || "light"
    );
  }
);

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
    return;
  },
  onUnload: () => {
    return;
  },
} as PageHandler);

addHandler({
  route: "#menu",
  element: "menu",
  onLoad: () => {
    if (window.fido.first_device_path) {
      window.fido.get_info().then((_) => {
        console.log(
          window.fido.first_device_path +
            " - " +
            _.aaguid.replace(
              /(.{8})(.{4})(.{4})(.{4})(.{12})/,
              "$1-$2-$3-$4-$5"
            )
        );

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
          showElementById("main-set-pin-disabled");
          hideElementById("main-change-pin-enabled");
          showElementById("main-change-pin-disabled");
        }

        showElementById("main-reset-enabled");
        hideElementById("main-reset-disabled");
      });
    } else {
      hideElementById("main-set-pin-enabled");
      showElementById("main-set-pin-disabled");
      hideElementById("main-change-pin-enabled");
      showElementById("main-change-pin-disabled");

      hideElementById("main-reset-enabled");
      showElementById("main-reset-disabled");
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
    (document.getElementById("change-pin-alert") as HTMLElement).style.display =
      "none";
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
    (document.getElementById("reset-alert") as HTMLElement).style.display =
      "none";
    (document.getElementById("resetConfirm") as HTMLInputElement).value = "";
  },
  onUnload: () => {
    return;
  },
} as PageHandler);

let current_device_list: FidoDeviceList;

window.addEventListener("DOMContentLoaded", () => {
  window.fido.init();

  window.addEventListener("hashchange", () => {
    onHashChange(window.location.hash);
  });

  window.fido.list_devices().then((device_list) => {
    current_device_list = device_list;

    listen("hid-watch", () => {
      window.fido.list_devices().then((device_list) => {
        const previous_device_list = current_device_list;
        current_device_list = device_list;

        const device_removed =
          window.fido.first_device_path &&
          !(window.fido.first_device_path in current_device_list.dev);

        if (device_removed) {
          window.fido.first_device_path = undefined;

          window.location.href = "#main";

          return;
        }

        const diff = current_device_list.dev.filter(
          (_) => !(_ in previous_device_list.dev)
        );

        if (diff.length >= 1) {
          window.fido.first_device_path = diff[0];

          window.location.href = "#menu";

          return;
        }
      });
    });
  });
});

declare global {
  interface Window {
    fido: Fido;

    showSetPinInput: () => void;

    showSetPinConfirm: () => void;

    showCurrentPin: () => void;

    showNewPin: () => void;

    showNewPinConfirm: () => void;
  }
}

window.fido = new Fido();

const showPin = (elementId: string) => () => {
  const x = <HTMLInputElement>document.getElementById(elementId);

  x.type = x.type === "password" ? "text" : "password";
};

window.showSetPinInput = showPin("setPinInput");

window.showSetPinConfirm = showPin("setPinConfirm");

window.showCurrentPin = showPin("currentPinInput");

window.showNewPin = showPin("newPinInput");

window.showNewPinConfirm = showPin("newPinConfirm");
