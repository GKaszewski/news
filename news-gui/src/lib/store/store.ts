import { StoreApi, create } from "zustand";
import { persist } from "zustand/middleware";
import { RssItem } from "../types";

export type AppSlice = {
  selectedText: string;
  setSelectedText: (text: string) => void;
  selectedRssItem: RssItem | null;
  setSelectedRssItem: (item: RssItem | null) => void;
};

export const createAppSlice: StoreSlice<AppSlice> = (set, get) => ({
  selectedText: "",
  selectedRssItem: null,
  setSelectedText: (text: string) => set({ selectedText: text }),
  setSelectedRssItem: (item: RssItem | null) => set({ selectedRssItem: item }),
});

export type StoreState = AppSlice;

export type StoreSlice<T> = (
  set: StoreApi<StoreState>["setState"],
  get: StoreApi<StoreState>["getState"]
) => T;

export const createPartializedState = (state: StoreState) => ({});

const useStore = create<StoreState>()(
  persist(
    (set, get) => ({
      ...createAppSlice(set, get),
    }),
    {
      name: "news-storage",
      partialize: (state) => createPartializedState(state),
      version: 1,
    }
  )
);

export default useStore;
