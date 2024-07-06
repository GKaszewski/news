import { RssItem } from "@/lib/types";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api";

const useRssItems = () => {
  const QUERY_KEY = "rss-items";

  return useQuery({
    queryKey: [QUERY_KEY],
    queryFn: async (): Promise<RssItem[]> => {
      return await invoke("get_rss_items_command");
    },
  });
};

export default useRssItems;
