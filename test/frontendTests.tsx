import React from 'react';
import { render, fireEvent, waitFor, screen } from '@testing-library/react';
import '@testing-1ibrary/jest-dom/extend-expect';
import { rest } from 'msw';
import { setupServer as setupMockServer } from 'msw/node';
import EventDisplayComponent from './EventComponent';

const localStorageMock = (function() {
  let store = {};
  return {
    getItem: function(key) {
      return store[key] || null;
    },
    setItem: function(key, value) {
      store[key] = value.toString();
    },
    clear: function() {
      store = {};
    },
  };
})();

Object.defineProperty(window, 'localStorage', {
  value: localStorageMock,
});

beforeAll(() => {});

describe('EventDisplayComponent with Caching Enhancements', () => {
  it('uses cache for event data to improve efficiency', async () => {
    window.localStorage.setItem('eventData', JSON.stringify({ eventData: 'cached event data' }));

    render(<EventDisplayComponent />);

    await waitFor(() => {
      expect(screen.getByTestId('fetched-data-display')).toHaveTextContent('cached event data');
    });
  });
});