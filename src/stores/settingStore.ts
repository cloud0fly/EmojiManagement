import { Store } from "@tauri-apps/plugin-store";

export type Settings = {
  memePath: string;
  autoScan: boolean;
  autoClose: boolean;
  alwaysOnTop: boolean;
  theme: "light" | "dark"; // 主题，未使用
};

// 默认配置
const DEFAULT_SETTINGS: Settings = {
  memePath: "",
  autoScan: true,
  autoClose: false,
  alwaysOnTop: false,
  theme: "light",
};

let storeInstance: Store | null = null;

// 获取或初始化 Store
async function getStore() {
  if (!storeInstance) {
    storeInstance = await Store.load("settings.json", {
      defaults: DEFAULT_SETTINGS,
      autoSave: 500, // 防抖处理
    });
  }
  return storeInstance;
}

// 加载配置
export async function loadSettings(): Promise<Settings> {
  const store = await getStore();
  const allEntries = await store.entries<any>();

  return Object.fromEntries(allEntries) as Settings;
}

// 更新单个配置
export async function updateSetting<K extends keyof Settings>(
  key: K,
  value: Settings[K],
) {
  const store = await getStore();
  await store.set(key, value);
}

// 重置配置
export async function resetSettings() {
  const store = await getStore();
  await store.reset();
}
