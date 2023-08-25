export type ErrorProps = {
    error: Error;
    reset: () => void;
}

export type gRPCError = {
  code: number
  details: string
  metadata: {
      "content-type": string[]
      "content-length": string[]
      date: string[]
  }
}
