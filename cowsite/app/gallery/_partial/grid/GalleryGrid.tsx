"use client";
import { Provider } from 'react-redux';
import store from '@/app/store';
import GalleryCard from './GalleryCard';
import { CowfileDescriptor } from '@/app/_api/cowserve';

type Props = {
  cowfiles: CowfileDescriptor[];
}

export default function GalleryGrid({ cowfiles }: Props) {
  return (
    <Provider store={store}>
      <div className="flex flex-wrap justify-center gap-5">
        {cowfiles.map((cowfile) => (
          <GalleryCard key={cowfile.id} cowfile={cowfile} />
        ))}
      </div>
    </Provider>
  );
}
