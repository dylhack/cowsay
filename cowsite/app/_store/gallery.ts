import { CowfileDescriptor } from "@/app/_api/cowserve/cowfiles_pb";
import { createSlice } from "@reduxjs/toolkit";

type GalleryState = {
    [key: string]: CowfileDescriptor.AsObject;
}

const initialState: GalleryState = {};

export const gallerySlice = createSlice({
    name: 'gallery',
    initialState,
    reducers: {
        addCowfile: (state, action) => {
            state[action.payload.getId()] = action.payload;
        },
        addCowfiles: (state, action) => {
            action.payload.forEach((c: CowfileDescriptor.AsObject) => {
                state[c.id] = c;
            });
        }
    },
});

export const { addCowfile, addCowfiles } = gallerySlice.actions;
export default gallerySlice.reducer;
