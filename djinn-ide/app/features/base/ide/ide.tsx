import Header from "../header";
import StatusBar from "../status-bar";

interface IdeProps {
  children: React.ReactNode;
}

export default function Ide({ children }: IdeProps) {
  return (
    <div className="grid h-dvh grid-rows-[auto_1fr_auto]">
      <Header />
      <main className="grid h-full min-h-0 overflow-hidden">{children}</main>
      <StatusBar />
    </div>
  );
}
