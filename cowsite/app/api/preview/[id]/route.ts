import { getCowfile, getPreview } from "@/app/api";
import { gRPCError } from "@/app/types";
import { NextResponse } from "next/server";


type Context = {
    params: {
        id: string;
    };
}

export async function GET(_req: Request, { params }: Context) {
    try {
        const cowfile = await getPreview(params.id);
        return new NextResponse(cowfile.getData_asU8(), {
            headers: {
                'Content-Type': 'image/webp',
                'Content-Disposition': `attachment; filename="${params.id}.webp"`,
            },
        });
    } catch (error) {
        const err = error as gRPCError;
        if (process.env.NODE_ENV === 'development') {
            return NextResponse.json({ error: err }, { status: 500 });
        }

        if (err.details.includes('no rows returned')) {
            return NextResponse.json({ message: 'Cowfile not found.' }, {
                status: 404,
            });
        } else {
            return NextResponse.json({ message: 'Internal server error.' }, {
                status: 500,
            });
        }
    }
}
