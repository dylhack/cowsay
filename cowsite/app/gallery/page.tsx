import Button from '../_components/Button';
import { getCowfiles } from '@/app/_api/cowserve';
import { chivoMono } from '../layout';
import GalleryGrid from './_partial/grid/GalleryGrid';
import styles from './styles.module.css';

export default async function Gallery() {
  const cowfiles = (await getCowfiles()).map(c => c.toObject());

  return (
    <main className={'flex flex-col ' + chivoMono.className}>
      <header className={styles.header}>
        <h1 className="text-white">Cowsite</h1>
        <div className='flex flex-row gap-52'>
          <Button style="danger">
            Clear selection
          </Button>
          <Button style="primary">
            Add to server
          </Button>
        </div>
      </header>
      <div className="font-mono">
        <GalleryGrid cowfiles={cowfiles} />
      </div>
    </main>
  );
}
