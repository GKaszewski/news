import { FeedUrl } from "@/lib/types";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api"

const useFeeds = () => {
    const QUERY_KEY = 'feeds';
    return useQuery({
        queryKey: [QUERY_KEY],
        queryFn: async (): Promise<FeedUrl[]> => {
           return await invoke('get_feeds');
        },
    });
}

export default useFeeds;