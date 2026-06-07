import Header from "../header";

interface IdeProps {
  children: React.ReactNode;
}

export default function Ide({ children }: IdeProps) {
  return (
    <div className="grid min-h-dvh grid-rows-[auto_1fr]">
      <Header />
      <main className="p-8 grid h-full">{children}</main>
    </div>
  );
}
