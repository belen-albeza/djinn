import Header from "../header";

interface IdeProps {
  children: React.ReactNode;
}

export default function Ide({ children }: IdeProps) {
  return (
    <>
      <Header />
      <main className="p-8">{children}</main>
    </>
  );
}
