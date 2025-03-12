import { createEffect, createSignal, For } from "solid-js";
import style from "./ComputedTextInput.module.css";

namespace OptionLabel {
  export type Props = {
    label: string,
    disabled?: boolean,
    active?: boolean,
    onClick?: () => void
  }
}

const OptionLabel = (props: OptionLabel.Props) => {
  return <div class={`${style.option} ${props.active ? "active" : ""} ${props.disabled ? "disabled" : ""}`} onClick={props.onClick}>
    {props.label}
  </div>
}

export namespace ComputedTextInput {  
  export type Props = {
    defaultOptions: ComputedOption[],
    onUpdate?: (value: string) => void
  }

  export type ComputedOption = {
    label: string,
    value: string|null;
  }
}

type State
  = { type: "custom" }
  | { type: "computed", index: number };

export const ComputedTextInput = (props: ComputedTextInput.Props) => {
  // custom input value is saved even when not displayed
  let [customValue, setCustomValue] = createSignal("");
  
  // component state
  let defaultState = (): State => {
    let idx = props.defaultOptions.findIndex(opt => opt.value !== null);
    if(idx >= 0) {
      return { type: "computed", index: idx };
    } else {
      return { type: "custom" };
    }
  }
  let [state, setState] = createSignal<State>(defaultState());

  // computed state
  let activeIdx = () => {
    let s = state();
    if(s.type === "custom") { return null; }
    return s.index;
  }

  // the externally-visible value of this input field
  let value = () => {
    let s = state();

    // handle custom input
    if(s.type === "custom") { return customValue(); }

    // handle default input, falling back to custom input if not available
    let opt = props.defaultOptions[s.index];
    if(!opt || opt.value === null) {
      return customValue(); 
    } else {
      return opt.value;
    }
  };

  // call onUpdate whenever value changes
  createEffect(() => {
    if(!props.onUpdate) { return; }
    props.onUpdate(value());
  });

  const optIsActive = (idx: number) => {
    return activeIdx() === idx;
  }

  const optHasValue = (idx: number): boolean => {
    let opt = props.defaultOptions[idx];
    if(opt === undefined) { return false; }
    return opt.value !== null;
  }

  const selectIndex = (idx: number) => {
    let opt = props.defaultOptions[idx];
    if(!opt || opt.value === null) { return; };

    setState({ type: "computed", index: idx });
  }

  const selectCustom = () => {
    setState({ type: "custom" });
  }

  return <div class={style.computedTextInput}>
    <div class={style.options}>
      <For each={props.defaultOptions}>
        {(option, idx) => (
          <OptionLabel
            disabled={!optHasValue(idx())}
            active={optIsActive(idx())}
            label={option.label}
            onClick={() => selectIndex(idx())}
          />
        )}
      </For>
      <OptionLabel
        active={state().type === "custom"}
        label="Custom"
        onClick={() => selectCustom()}
      />
    </div>
    <div class={style.input}>
      <input
        type="text"
        placeholder="Custom Input"
        value={value()}
        onInput={(e) => {
          e.preventDefault();
          setCustomValue(e.currentTarget.value);
          selectCustom();
        }}
      />
    </div>
  </div>
}