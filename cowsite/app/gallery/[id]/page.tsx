import { getCowfile } from "@/app/_api/cowserve";
import Download from "./_partial/Download";
import Card from "../_partial/Card";

type Props = {
    params: {
        id: string;
    }
}

export default async function Chara({ params }: Props) {
    const cowfile = await getCowfile(params.id);
    const download = (
        <Download id={cowfile.id} />
    );

    return (
        <Card cowfile={cowfile} actions={[download]} />
    );
}
