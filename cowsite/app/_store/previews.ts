import { createSlice } from "@reduxjs/toolkit";
import { AppState } from "../store";

type PreviewState = {
    imgUrl?: string;
}
const initialState: PreviewState = {};

export const previewSlice = createSlice({
    name: 'previews',
    initialState,
    reducers: {
        setPreview: (state, action) => {
            state.imgUrl = action.payload;
        },
    },
});

export const { setPreview } = previewSlice.actions;
export const previewState = (state: AppState) => state.gallery.imgUrl;
export default previewSlice.reducer;
