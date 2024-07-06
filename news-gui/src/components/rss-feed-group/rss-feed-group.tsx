import {
  Collapsible,
  CollapsibleTrigger,
  CollapsibleContent,
} from '@/components/ui/collapsible';
import { RssItem } from '@/lib/types';

interface Props {
  groupedRssItems: Record<string, RssItem[]>;
  handleItemClick: (item: RssItem) => void;
  iconUrl: string;
  groupName: string;
}

const RssFeedGroup = ({
  groupedRssItems,
  handleItemClick,
  iconUrl,
  groupName,
}: Props) => {
  return (
    <Collapsible>
      <CollapsibleTrigger className="inline-flex items-center font-semibold">
        <img
          className="object-cover object-center w-8 h-8 dark:invert"
          src={iconUrl}
        />
        {groupName}
      </CollapsibleTrigger>
      <CollapsibleContent>
        <ul className="flex flex-col gap-2">
          {groupedRssItems[groupName]?.map((item) => (
            <li
              key={item.title}
              onClick={() => handleItemClick(item)}
              className="transition-colors duration-300 cursor-pointer hover:underline hover:text-blue-500"
            >
              {item.title}
            </li>
          ))}
        </ul>
      </CollapsibleContent>
    </Collapsible>
  );
};

export default RssFeedGroup;
