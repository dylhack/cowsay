import { createSlice } from "@reduxjs/toolkit";
import { AppState } from "../store";

type AddCharasState = string[];
const initialState: AddCharasState = [];

export const addCharaSlice = createSlice({
    name: 'addcharas',
    initialState,
    reducers: {
        selectChara(state, action) {
            state.push(action.payload);
        },
        deselectChara(state, action) {
            state.splice(state.indexOf(action.payload), 1);
        }
    },
});

export const { selectChara, deselectChara } = addCharaSlice.actions;
export const selectedCharas = (state: AppState) => state.addcharas;
export default addCharaSlice.reducer;
