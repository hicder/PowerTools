import { Fragment } from "react";
import { Component } from "react";
import {
    ToggleField,
    SliderField,
    PanelSectionRow,
    staticClasses,
    Field,
    Dropdown,
    SingleDropdownOption,
} from "decky-frontend-lib";
import * as backend from "../backend";
import { tr } from "usdpl-front";
import {
    LIMITS_INFO,
    SLOW_PPT_GPU,
    FAST_PPT_GPU,
    TDP,
    PRESET_MODE_GPU,
    CLOCK_MIN_GPU,
    CLOCK_MAX_GPU,
} from "../consts";
import { set_value, get_value } from "usdpl-front";

export class Gpu extends Component<backend.IdcProps> {
    constructor(props: backend.IdcProps) {
        super(props);
        this.state = {
            reloadThingy: "/shrug",
        };
    }

    render() {
        const reloadGUI = (x: string) => this.setState({ reloadThingy: x });

        const performanceDropdown: SingleDropdownOption[] = [
            {data: 0, label: <span>Silent 10W</span> }, 
            {data: 1, label: <span>Peformance 15W</span> }, 
            {data: 5, label: <span>Performance 20W</span>},
            {data: 2, label: <span>Turbo 25W</span>}, 
            {data: 3, label: <span>Turbo 30W</span>}, 
            {data: 4, label: <span>Manual</span>}
        ];

        const labels : string [] = ["Silent 10W", "Performance 15W", "Turbo 25W", "Turbo 30W", "Manual", "Performance 20W"];

        return (<Fragment>
            {/* GPU */}
            <div className={staticClasses.PanelSectionTitle}>
                {tr("GPU")}
            </div>
            {((get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.fast_ppt_limits != null || (get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.slow_ppt_limits != null) && <PanelSectionRow>
                <ToggleField
                    checked={get_value(SLOW_PPT_GPU) != null || get_value(FAST_PPT_GPU) != null}
                    label={tr("Thermal Power (TDP) Limit")}
                    description={tr("Limits processor power for less total power")}
                    onChange={(value: boolean) => {
                        if (value) {
                            if ((get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.slow_ppt_limits != null) {
                                set_value(SLOW_PPT_GPU, (get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.slow_ppt_limits!.max);

                                // Set it to midpoint
                                set_value(TDP, (get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.slow_ppt_limits!.max / 2000);
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
                    <Field
                    label={tr("Performance Preset")}
                    >
                    <Dropdown
                        menuLabel={tr("Performance")}
                        rgOptions={performanceDropdown}
                        selectedOption={performanceDropdown.find((val: SingleDropdownOption, _index, _arr) => {
                            backend.log(backend.LogLevel.Info, "Checking selected option: " + val.data.toString());

                            // Default to Performance 15W
                            if (get_value(PRESET_MODE_GPU) == null) {
                                backend.log(backend.LogLevel.Info, "Preset mode is null");
                                return val.data == 1;
                            } else {
                                backend.log(backend.LogLevel.Info, "Preset mode is " + get_value(PRESET_MODE_GPU).toString());
                            }
                            return val.data == get_value(PRESET_MODE_GPU);
                        })}
                        strDefaultLabel={get_value(PRESET_MODE_GPU) == null ? "Performance 15W" : labels[get_value(PRESET_MODE_GPU)]}
                        onChange={(elem: SingleDropdownOption) => {
                            backend.log(backend.LogLevel.Debug, "Performance dropdown selected " + elem.data.toString());
                            backend.resolve(backend.setPreset(elem.data), (_value) => {});

                            // Set this outside instead of in the callback because it's faster for UI.
                            backend.log(backend.LogLevel.Info, "Preset mode is now " + elem.data.toString());
                            set_value(PRESET_MODE_GPU, elem.data);

                            switch (elem.data) {
                                case 0:
                                    backend.resolve(backend.setGpuPptTdp(10000, 17000, 14000), (limits: number[]) => {
                                        set_value(TDP, limits[0]/1000);
                                        set_value(FAST_PPT_GPU, limits[1]);
                                        set_value(SLOW_PPT_GPU, limits[2]);
                                    });
                                    break;
                                case 1:
                                    backend.resolve(backend.setGpuPptTdp(15000, 17000, 15000), (limits: number[]) => {
                                        set_value(TDP, limits[0]/1000);
                                        set_value(FAST_PPT_GPU, limits[1]);
                                        set_value(SLOW_PPT_GPU, limits[2]);
                                    });
                                    break;
                                case 2:
                                    backend.resolve(backend.setGpuPptTdp(25000, 35000, 30000), (limits: number[]) => {
                                        set_value(TDP, limits[0]/1000);
                                        set_value(FAST_PPT_GPU, limits[1]);
                                        set_value(SLOW_PPT_GPU, limits[2]);
                                    });
                                    break;
                                case 5:
                                    backend.resolve(backend.setGpuPptTdp(20000, 22000, 20000), (limits: number[]) => {
                                        set_value(TDP, limits[0]/1000);
                                        set_value(FAST_PPT_GPU, limits[1]);
                                        set_value(SLOW_PPT_GPU, limits[2]);
                                    });
                                    break;
                                case 3:
                                    backend.resolve(backend.setGpuPptTdp(30000, 53000, 43000), (limits: number[]) => {
                                        set_value(TDP, limits[0]/1000);
                                        set_value(FAST_PPT_GPU, limits[1]);
                                        set_value(SLOW_PPT_GPU, limits[2]);
                                    });
                                    break;
                                case 4:
                                    break;
                            }

                            reloadGUI("performance " + (new Date()).getTime().toString());
                        }}
                    />
                    </Field>
            </PanelSectionRow>
            <PanelSectionRow>
                {get_value(TDP) != null && get_value(PRESET_MODE_GPU) == 4 && <SliderField
                    label={tr("Watts")}
                    value={get_value(TDP)}
                    max={30}
                    min={7}
                    step={1}
                    showValue={true}
                    disabled={get_value(TDP) == null || get_value(PRESET_MODE_GPU) != 4}
                    onChange={(tdp: number) => {
                        backend.log(backend.LogLevel.Debug, "TDP is now " + tdp.toString());
                        const oldTDP = get_value(TDP);
                        const newTdp = tdp;
                        if (newTdp != oldTDP) {
                            backend.resolve(backend.setGpuPptTdp(newTdp * 1000, newTdp * 1000 + 2000, newTdp * 1000),
                                (limits: number[]) => {
                                    set_value(TDP, limits[0] / 1000);
                                    set_value(FAST_PPT_GPU, limits[1]);
                                    set_value(SLOW_PPT_GPU, limits[2]);
                                    reloadGUI("GPUTDP");
                                });
                        }
                    }}
                />}
            </PanelSectionRow>
            {((get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.clock_min_limits != null || (get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.clock_max_limits != null) && <PanelSectionRow>
                <ToggleField
                checked={get_value(CLOCK_MIN_GPU) != null || get_value(CLOCK_MAX_GPU) != null}
                label={tr("Frequency Limits")}
                description={tr("Set bounds on clock speed")}
                onChange={(value: boolean) => {
                    backend.log(backend.LogLevel.Info, "CHANGED GPU CLOCK LIMIT: " + value.toString());
                    if (value) {
                        let clock_min_limits = (get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.clock_min_limits;
                        let clock_max_limits = (get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.clock_max_limits;
                        if (clock_min_limits != null) {
                            set_value(CLOCK_MIN_GPU, clock_min_limits.min);
                        }
                        if (clock_max_limits != null) {
                            set_value(CLOCK_MAX_GPU, clock_max_limits.max);
                        }
                        reloadGUI("GPUFreqToggle");
                    } else {
                        set_value(CLOCK_MIN_GPU, null);
                        set_value(CLOCK_MAX_GPU, null);
                        backend.resolve(backend.unsetGpuClockLimits(), (_: any[]) => {
                            reloadGUI("GPUUnsetFreq");
                        });
                    }
                }}
                />
            </PanelSectionRow>}
            <PanelSectionRow>
                { get_value(CLOCK_MIN_GPU) != null && <SliderField
                label={tr("Minimum (MHz)")}
                value={get_value(CLOCK_MIN_GPU)}
                max={(get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.clock_min_limits!.max}
                min={(get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.clock_min_limits!.min}
                step={(get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.clock_step}
                showValue={true}
                disabled={get_value(CLOCK_MIN_GPU) == null}
                onChange={(val: number) => {
                    backend.log(backend.LogLevel.Info, "GPU Clock Min is now " + val.toString());
                    const valNow = get_value(CLOCK_MIN_GPU);
                    const maxNow = get_value(CLOCK_MAX_GPU);
                    if (val != valNow && ((maxNow != null && val <= maxNow) || maxNow == null)) {
                        backend.resolve(backend.setGpuClockLimits(val, get_value(CLOCK_MAX_GPU)),
                                        (limits: number[]) => {
                            set_value(CLOCK_MIN_GPU, limits[0]);
                            set_value(CLOCK_MAX_GPU, limits[1]);
                            reloadGUI("GPUMinClock");
                        });
                    }
                }}
                />}
            </PanelSectionRow>
            <PanelSectionRow>
                {get_value(CLOCK_MAX_GPU) != null && <SliderField
                label={tr("Maximum (MHz)")}
                value={get_value(CLOCK_MAX_GPU)}
                max={(get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.clock_max_limits!.max}
                min={(get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.clock_max_limits!.min}
                step={(get_value(LIMITS_INFO) as backend.SettingsLimits).gpu.clock_step}
                showValue={true}
                disabled={get_value(CLOCK_MAX_GPU) == null}
                onChange={(val: number) => {
                    backend.log(backend.LogLevel.Info, "GPU Clock Max is now " + val.toString());
                    const valNow = get_value(CLOCK_MAX_GPU);
                    const minNow = get_value(CLOCK_MIN_GPU);
                    if (val != valNow && ((minNow != null && val >= minNow) || minNow == null)) {
                        backend.resolve(backend.setGpuClockLimits(get_value(CLOCK_MIN_GPU), val),
                                        (limits: number[]) => {
                            set_value(CLOCK_MIN_GPU, limits[0]);
                            set_value(CLOCK_MAX_GPU, limits[1]);
                            reloadGUI("GPUMaxClock");
                        });
                    }
                }}
                />}
            </PanelSectionRow>
        </Fragment>);
    }
}
