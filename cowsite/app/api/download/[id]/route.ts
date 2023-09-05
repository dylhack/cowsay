import { getCowdata, getCowfile } from "@/app/_api/cowserve";
import { gRPCError } from "@/app/types";
import { NextResponse } from "next/server";


type Context = {
    params: {
        id: string;
    };
}

export async function GET(_req: Request, { params }: Context) {
    try {
        const cowfile = await getCowdata(params.id);
        const data = Buffer.from(cowfile.data, 'base64').toString('utf-8');
        return new NextResponse(data, {
            headers: {
                'Content-Type': 'text/plain',
                'Content-Disposition': `attachment; filename="${params.id}.cow"`,
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
