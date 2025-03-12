import { createResource, Show, Switch, Match, Accessor, JSX, For } from "solid-js";
import { Api } from "../../api";
import "./ManageFeeds.css";


type ResultOk<T,E>  = Extract<Api.Result<T,E>, { status: "ok" }>;
type ResultErr<T,E> = Extract<Api.Result<T,E>, { status: "error" }>;

const isOk = <T,E>(result: Api.Result<T, E>): result is ResultOk<T,E> => {
  return result.status === "ok";
}
const isErr = <T,E>(result: Api.Result<T, E>): result is ResultErr<T,E> => {
  return result.status === "ok";
}

////////////////////////////////////////////////////////////////////////////////

type Predicate<S extends T, T> = (e:T) => e is S;

function matches<S extends T, T>(e:T, predicate: Predicate<S, T>):S|false {
  return predicate(e) ? e : false;
}

namespace Case {
  export type Props<S extends T,T> = {
    on: T,
    when: Predicate<S,T>,
    children: (item: Accessor<NonNullable<S>>) => JSX.Element
  }
}

const Case = <S extends T, T>(props: {
  on: T,
  when: Predicate<S,T>,
  children: (item: Accessor<NonNullable<S>>) => JSX.Element
}) => {
  return <Match when={matches(props.on, props.when)}>
    {props.children}
  </Match>;
}

const Case3 = <S extends T, T>(props: Case.Props<S,T>) => {
  return <Match when={matches(props.on, props.when)}>
    {props.children}
  </Match>;
}

const Case2 = <S extends T, T>(
  on: T,
  when: Predicate<S,T>,
  children: (item: Accessor<NonNullable<S>>) => JSX.Element
) => {
  return <Match when={matches(on, when)}>
    {children}
  </Match>;
}

export const ManageFeeds = () => {
  let [feeds, setFeeds] = createResource(async () => {
    let result = await Api.commands.readAllFeeds();
    return result;
  });
  
  return <div>
    <h1>Manage Feeds</h1>
    <Show when={feeds()}>
      <Switch fallback={<div></div>}>
        <Match when={matches(feeds()!, isOk)}>
          {(result) => {
            return <For each={result().data}>
              {(feed) => (<div>
                {feed.title}
              </div>)}
              </For>;
          }}
        </Match>
        <Match when={matches(feeds()!, isErr)}>
          {(result) => {
            return <div>{result().error}</div>
          }}
        </Match>
      </Switch>
    </Show>
  </div>;
}