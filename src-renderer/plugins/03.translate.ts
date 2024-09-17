import { invoke, type InvokeArgs } from "@tauri-apps/api/core";

interface TextOptions {
  text: string;
  from: string;
  to: string;
  apiType: string;
}
interface TextResult {
  detected_source_language: string | null;
  text: string[];
}

interface LanguagesOptions {
  apiType: string;
  displayLanguageCode: string;
}

interface Languages {
  languages: Array<{
    display_name: string;
    language_code: string;
  }>;
}

interface DetectOptions {
  apiType: string;
  text: string;
}

interface Detect {
  confidence: number;
  language_code: string;
}

interface SpeechOptions {
  text: string;
  lang: string;
}

interface Speech {
  data: any;
}

interface Img2text {
  detected_text: string;
}

export class Translate {
  async translate_text(opt: TextOptions): Promise<TextResult> {
    return await invoke("translate_text", { ...opt });
  }

  async translate_languages(opt: LanguagesOptions): Promise<Languages> {
    const result: Languages = await invoke("translate_languages", {
      ...opt,
    });
    return result;
  }

  async translate_detect(opt: DetectOptions): Promise<Detect> {
    const result: Detect = await invoke("translate_detect", {
      ...opt,
    });
    return result;
  }

  async translate_speech(opt: SpeechOptions): Promise<string> {
    const result: Speech = await invoke("translate_speech", { ...opt });
    const byteArray = new Uint8Array(result.data);
    const audioBlob = new Blob([byteArray], { type: "audio/mpeg" });
    return URL.createObjectURL(audioBlob);
  }

  async translate_img2text(opt: { img: ArrayBuffer }): Promise<Img2text> {
    const imgBytes = Array.from(new Uint8Array(opt.img));
    return await invoke("translate_img2text", { apiType: "", imgBytes });
  }
}

export default defineNuxtPlugin({
  name: "translate",
  async setup(app) {
    const translate = new Translate();
    app.provide("translate", translate);
  },
});
