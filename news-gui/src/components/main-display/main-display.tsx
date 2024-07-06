import useStore from "@/lib/store/store";

const MainDisplay = () => {
  const selectedRssItem = useStore((state) => state.selectedRssItem);

  return (
    <main className="w-full h-full">
      {selectedRssItem && (
        <>
          <h1 className="text-xl font-semibold text-pretty">
            {selectedRssItem.title}
          </h1>
          <p className="font-sans text-lg text-pretty ">
            {selectedRssItem.description}
          </p>
          <a
            href={selectedRssItem.link}
            target="_blank"
            rel="noreferrer"
            className="underline transition-colors cursor-pointer hover:text-blue-500"
          >
            Read More
          </a>
        </>
      )}
    </main>
  );
};

export default MainDisplay;
