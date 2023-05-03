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

export class Fido {
  first_device_path: string | undefined;

  init() {
    console.log("fido_init");
    invoke("fido_init");
  }

  list_devices(): Promise<FidoDeviceList> {
    console.log("fido_list_devices");
    return invoke("fido_list_devices");
  }

  get_info(): Promise<FidoGetInfoResponse> {
    console.log("fido_get_info");
    return invoke("fido_get_info", {
      parameters: { dev: this.first_device_path } as FidoGetInfoCommand,
    });
  }

  invoke_set_pin(parameters: FidoSetPinCommand): Promise<FidoSetPinResponse> {
    console.log("fido_set_pin");
    return invoke("fido_set_pin", { parameters: parameters });
  }

  setPin(newPin: string): Promise<FidoSetPinResponse> {
    console.log(this.first_device_path);
    return this.invoke_set_pin({
      dev: this.first_device_path,
      newPin: newPin,
    } as FidoSetPinCommand);
  }

  invoke_change_pin(
    parameters: FidoChangePinCommand
  ): Promise<FidoChangePinResponse> {
    console.log("fido_change_pin");
    return invoke("fido_change_pin", { parameters: parameters });
  }

  changePin(newPin: string, oldPin: string): Promise<FidoChangePinResponse> {
    console.log(this.first_device_path);
    return this.invoke_change_pin({
      dev: this.first_device_path,
      newPin: newPin,
      oldPin: oldPin,
    } as FidoChangePinCommand);
  }

  invoket_reset(parameters: FidoResetCommand): Promise<FidoResetResponse> {
    return invoke("fido_reset", { parameters: parameters });
  }

  reset(): Promise<FidoResetResponse> {
    console.log(this.first_device_path);
    return this.invoket_reset({
      dev: this.first_device_path,
    } as FidoResetCommand);
  }
}
