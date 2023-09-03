import { Fragment } from "react";
import {Component} from "react";
import {
  ToggleField,
  SliderField,
  PanelSectionRow,
  staticClasses,
} from "decky-frontend-lib";
import * as backend from "../backend";
import { tr } from "usdpl-front";
import {
    LIMITS_INFO,
    SLOW_PPT_GPU,
    FAST_PPT_GPU,
    TDP,
} from "../consts";
import { set_value, get_value} from "usdpl-front";

export class Gpu extends Component<backend.IdcProps> {
    constructor(props: backend.IdcProps) {
        super(props);
        this.state = {
            reloadThingy: "/shrug",
        };
    }

    render() {
        const reloadGUI = (x: string) => this.setState({reloadThingy: x});
        return (<Fragment>
                {/* GPU */}
            <div className={staticClasses.PanelSectionTitle}>
                {tr("GPU")}
            </div>
            { ((get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.fast_ppt_limits != null ||(get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.slow_ppt_limits != null) && <PanelSectionRow>
                <ToggleField
                checked={get_value(SLOW_PPT_GPU) != null || get_value(FAST_PPT_GPU) != null}
                label={tr("Thermal Power (TDP) Limit")}
                description={tr("Limits processor power for less total power")}
                onChange={(value: boolean) => {
                    if (value) {
                        if ((get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.slow_ppt_limits != null) {
                            set_value(SLOW_PPT_GPU, (get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.slow_ppt_limits!.max);

                            // Set it to midpoint
                            set_value(TDP, (get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.slow_ppt_limits!.max/2000);
                        }

                        if ((get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.fast_ppt_limits != null) {
                            set_value(FAST_PPT_GPU, (get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.fast_ppt_limits!.max);
                        }
                        reloadGUI("GPUPPTToggle");
                    } else {
                        set_value(SLOW_PPT_GPU, null);
                        set_value(FAST_PPT_GPU, null);
                        set_value(TDP, null);
                        backend.resolve(backend.unsetGpuPpt(), (_: any[]) => {
                            reloadGUI("GPUUnsetPPT");
                        });
                    }
                }}
                />
            </PanelSectionRow>}
            <PanelSectionRow>
                {get_value(TDP) != null && <SliderField
                    label={tr("Watts")}
                    value={get_value(TDP)}
                    max={(get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.slow_ppt_limits!.max / 1000}
                    min={(get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.slow_ppt_limits!.min / 1000}
                    step={1}
                    showValue={true}
                    disabled={get_value(TDP) == null}
                    onChange={(tdp: number) => {
                        backend.log(backend.LogLevel.Debug, "TDP is now " + tdp.toString());
                        const oldTDP = get_value(TDP);
                        const newTdp = tdp;
                        if (newTdp != oldTDP) {
                            backend.resolve(backend.setGpuPpt(newTdp * 1000 + 2000, newTdp * 1000),
                                (limits: number[]) => {
                                    set_value(FAST_PPT_GPU, limits[0]);
                                    set_value(SLOW_PPT_GPU, limits[1]);
                                    set_value(TDP, limits[1] / 1000);
                                    reloadGUI("GPUTDP");
                                });
                        }
                    }}
                />}
            </PanelSectionRow>
            </Fragment>);
    }
}
