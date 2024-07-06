import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"
import { RssItem } from "./types";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export const getGroupedRssItemsBySource = (rssItems: RssItem[] ) => {
  if (!rssItems) {
    return {};
  }

  return rssItems.reduce((acc, rssItem) => {
    if (!acc[rssItem.source]) {
      acc[rssItem.source] = [];
    }

    acc[rssItem.source].push(rssItem);

    return acc;
  }, {} as Record<string, RssItem[]>);
};