"use client"
import MoreInfo from "./MoreInfo";
import Claim from "./Claim";
import { useSelector } from "react-redux";
import { selectedCharas } from "@/app/_store/addchara";
import { CowfileDescriptor } from "@/app/_api/cowserve";
import Card from "../Card";

type Props = {
    cowfile: CowfileDescriptor;
}

export default function GalleryCard({ cowfile }: Props) {
    const selected = useSelector(selectedCharas);
    const checked = selected.find((id) => id === cowfile.id) !== undefined;
    const actions = [];
    if (!checked) actions.push(<MoreInfo cid={cowfile.id} />);
    actions.push(<Claim />);

    return (
        <Card cowfile={cowfile} actions={actions}/>
    );
}
