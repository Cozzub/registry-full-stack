// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { HeadCell } from "./HeadCell";
import type { RowCell } from "./RowCell";

export interface Registry { uuid: string, owner: string, name: string, columns: Array<HeadCell>, rows: Array<Record<string, RowCell>>, }