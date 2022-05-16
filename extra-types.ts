import * as t from 'io-ts'

export const Operation = t.keyof({
  INSERT: null,
  UPDATE: null,
  DELETE: null,
})

export const SourceReference = t.type({
  table: t.string,
})

export const GammasoftEvent = t.type({
  eventType: Operation,
  sourceReference: SourceReference,
  data: SupportedDbEvents
})

export const CdcEvent = t.type({
  type: "DBUpdate",
  table: t.string,
  columns: t.intersection([
    t.type({
      OPERATION: Operation
    }),
    SupportedDbEvents
  ])
})

export const SupportedEvents = t.union([
  GammasoftEvent,
  CdcEvent
])

export type SupportedEvents = t.TypeOf<typeof SupportedEvents>