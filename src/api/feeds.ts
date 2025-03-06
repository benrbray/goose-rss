import { invoke } from "@tauri-apps/api/core";

export async function greet(name: string): Promise<string> {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  let result = await invoke("greet", { name });
  return result as string;
}