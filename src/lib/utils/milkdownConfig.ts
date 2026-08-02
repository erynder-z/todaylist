import { createParser } from "@milkdown/plugin-highlight/lowlight";
import { common, createLowlight } from "lowlight";

export const lowlight = createLowlight(common);
export const parser = createParser(lowlight);
