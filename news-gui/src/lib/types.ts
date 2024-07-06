export type FeedUrl = {
  id: number;
  url: string;
  name: string;
};

export type RssItem = {
  title: string;
  link: string;
  description: string;
  pubDate: string;
  source: string;
};
