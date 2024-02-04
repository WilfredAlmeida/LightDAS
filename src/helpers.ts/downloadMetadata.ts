import axios from "axios";

export const downloadMetadata = async (url: string) => {
  try {
    const { data } = await axios.get(url);

    if (!data) {
      return null;
    }

    return data;
  } catch (error) {
    console.error(error);
    return null;
  }
};
