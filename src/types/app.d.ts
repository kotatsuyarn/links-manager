interface IListGroup {
  uuid: uuid;
  name: string;
  position: number;
  lists?: IList[];
}

interface IList {
  uuid: uuid;
  name: string;
  // description: string;
  position: number;
  listGroupUuid?: uuid;
  // tasks?: uuid[];
}

interface ITask {
  uuid: uuid;
  title: string;
  description: string;
  url: string;
  position: number;
  listUuid: uuid;
}

type uuid = string;
