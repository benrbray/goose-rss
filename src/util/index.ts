import { Api } from "../api";

export type Predicate<S extends T, T> = (e:T) => e is S;

export function matches<S extends T, T>(e:T, predicate: Predicate<S, T>):S|false {
  return predicate(e) ? e : false;
}

export type ResultOk<T,E>  = Extract<Api.Result<T,E>, { status: "ok" }>;
export type ResultErr<T,E> = Extract<Api.Result<T,E>, { status: "error" }>;

export const isOk = <T,E>(result: Api.Result<T, E>): result is ResultOk<T,E> => {
  return result.status === "ok";
}

export const isErr = <T,E>(result: Api.Result<T, E>): result is ResultErr<T,E> => {
  return result.status === "ok";
}
