import { cx } from "class-variance-authority";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import Sidebar from "@/components/sidebar";

const Layout = ({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) => {
  return (
    <div className={cx(className, "w-full min-h-screen max-h-screen")}>
      <ResizablePanelGroup
        direction="horizontal"
        className="w-full h-full min-h-screen"
      >
        <ResizablePanel
          className="min-w-40 w-40 max-w-[400px]"
          defaultSize={20}
        >
          <Sidebar />
        </ResizablePanel>
        <ResizableHandle />
        <ResizablePanel>{children}</ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
};

export default Layout;
