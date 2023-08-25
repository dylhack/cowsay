import { getCowfiles } from './api';
import Cowfile from './components/Cowfile';


export default async function Home() {
  const cowfiles = await getCowfiles();

  return (
    <main>
      <h1>Cowsite</h1>
      {/* tailwind css */}
      <div className="flex flex-wrap justify-center gap-5">
        {cowfiles.map((cowfile) => (
          <Cowfile key={cowfile.getName()} {...cowfile.toObject()} />
        ))}
      </div>
    </main>
  )
}
