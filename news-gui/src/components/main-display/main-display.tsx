import useStore from "@/lib/store/store";

const MainDisplay = () => {
  const selectedRssItem = useStore((state) => state.selectedRssItem);

  return (
    <main className="flex flex-col items-center w-full h-full font-serif bg-gray-100 text-pretty dark:bg-gray-950">
      <div className="h-full p-8 w-[80ch]">
        {selectedRssItem && (
          <>
            <h1 className="mb-8 font-semibold text-center md:text-lg lg:text-2xl">
              {selectedRssItem.title}
            </h1>
            <p className="text-base break-words lg:text-lg">
              {selectedRssItem.description}
            </p>
            <a
              href={selectedRssItem.link}
              target="_blank"
              rel="noreferrer"
              className="transition-colors cursor-pointer sunderline hover:text-blue-500"
            >
              Read More
            </a>
          </>
        )}
      </div>
    </main>
  );
};

export default MainDisplay;
