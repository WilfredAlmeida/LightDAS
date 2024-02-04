import axios from "axios";

export const downloadMetadata = async (url: string) => {
  const { data } = await axios.get(url);

  if (!data) {
    return null;
  }

  return data;
};
