import axios from 'axios';

export let client = axios.create({
    baseURL: '/api/v1',
    timeout: 30000,
});

client.interceptors.response.use(
    (response) => {
        return response;
    },
    (error) => {
        console.log(error);
        return Promise.reject(error);
    }
)

type Category = {
    id: number,
    name: string,
};

type CategoryList = { categories: Category[] };

type Word = {
    id: number,
    word: string,
};

type WordList = { words: Word[] };

export async function listCategories(): Promise<CategoryList> {
    return client.get('words').then((r) => r.data);
}

export async function listWords(category_id: number): Promise<WordList> {
    return client.get(`words/${category_id}`).then((r) => r.data);
}
