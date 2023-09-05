import { Action, ThunkAction, configureStore } from '@reduxjs/toolkit'
import { gallerySlice } from './_store/gallery';
import { previewSlice } from './_store/previews';
import { addCharaSlice } from './_store/addchara';


const makeStore = () => configureStore({
  reducer: {
    [gallerySlice.name]: gallerySlice.reducer,
    [previewSlice.name]: previewSlice.reducer,
    [addCharaSlice.name]: addCharaSlice.reducer,
  },
  devTools: true
});

export type AppStore = ReturnType<typeof makeStore>;
export type AppState = ReturnType<AppStore["getState"]>;
export type AppThunk<ReturnType = void> = ThunkAction<
  ReturnType,
  AppState,
  unknown,
  Action
>;

export default makeStore();
