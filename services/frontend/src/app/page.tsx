import { TranslatePrompt } from "@/components/translate_prompt";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-start p-24 space-y-4">
      <div className="flex flex-col w-full h-20 items-center justify-center text-blue-300">
        <p className="text-4xl font-sans italic">Nonsense Translator</p>
      </div>
      <TranslatePrompt />
    </main>
  );
}
