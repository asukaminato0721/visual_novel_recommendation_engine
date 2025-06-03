import init, { WasmRecommender } from './pkg/visual_novel_recommendation_engine.js';

class RecommendationApp {
    constructor() {
        this.recommender = null;
        this.dataLoaded = {
            titles: false,
            tags: false,
            votes: false
        };
        this.initializeEventListeners();
    }

    async initialize() {
        try {
            await init();
            this.recommender = new WasmRecommender();
            console.log('WASM module initialized successfully');
            
            // Setup file upload handlers
            this.setupFileUploadHandlers();
            
        } catch (error) {
            console.error('Failed to initialize WASM module:', error);
            this.showError('Failed to initialize the recommendation engine. Please refresh the page.');
        }
    }

    setupFileUploadHandlers() {
        // VN Titles file upload
        document.getElementById('vnTitlesFile').addEventListener('change', (event) => {
            this.handleFileUpload(event, 'vnTitles');
        });

        // Tags file upload
        document.getElementById('tagsFile').addEventListener('change', (event) => {
            this.handleFileUpload(event, 'tags');
        });

        // Votes file upload
        document.getElementById('votesFile').addEventListener('change', (event) => {
            this.handleFileUpload(event, 'votes');
        });
    }

    async handleFileUpload(event, dataType) {
        const file = event.target.files[0];
        if (!file) return;

        const statusId = dataType === 'vnTitles' ? 'vnTitlesStatus' : 
                        dataType === 'tags' ? 'tagsStatus' : 'votesStatus';

        try {
            this.updateDataStatus(statusId, 'loading', 'Reading file...');
            this.showUploadProgress(true);

            const fileContent = await this.readFileAsText(file);
            
            let success = false;
            switch (dataType) {
                case 'vnTitles':
                    success = this.recommender.load_vn_titles(fileContent);
                    if (success) {
                        this.dataLoaded.titles = true;
                        this.updateDataStatus(statusId, 'success', `Loaded (${file.name})`);
                    }
                    break;
                case 'tags':
                    success = this.recommender.load_tags(fileContent);
                    if (success) {
                        this.dataLoaded.tags = true;
                        this.updateDataStatus(statusId, 'success', `Loaded (${file.name})`);
                    }
                    break;
                case 'votes':
                    success = this.recommender.load_votes(fileContent);
                    if (success) {
                        this.dataLoaded.votes = true;
                        this.updateDataStatus(statusId, 'success', `Loaded (${file.name})`);
                    }
                    break;
            }

            if (!success) {
                throw new Error(`Failed to process ${dataType} data`);
            }

            // Update progress based on loaded data
            this.updateOverallProgress();
            
            // Check if we can enable recommendations
            this.updateGetRecommendationsButton();

        } catch (error) {
            console.error(`Error loading ${dataType}:`, error);
            this.updateDataStatus(statusId, 'error', 'Failed to load');
            this.showError(`Failed to load ${dataType} file: ${error.message}`);
        } finally {
            this.showUploadProgress(false);
        }
    }

    readFileAsText(file) {
        return new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = (e) => resolve(e.target.result);
            reader.onerror = (e) => reject(new Error('Failed to read file'));
            reader.readAsText(file);
        });
    }

    updateOverallProgress() {
        const loadedCount = Object.values(this.dataLoaded).filter(Boolean).length;
        const totalCount = Object.keys(this.dataLoaded).length;
        const percentage = (loadedCount / totalCount) * 100;
        
        this.updateProgress(percentage, `${loadedCount}/${totalCount} files loaded`);
        
        if (loadedCount === totalCount) {
            setTimeout(() => {
                this.showUploadProgress(false);
            }, 1000);
        }
    }

    updateDataStatus(elementId, status, text) {
        const element = document.getElementById(elementId);
        if (!element) return;
        
        let iconClass, textClass;
        switch (status) {
            case 'loading':
                iconClass = 'fas fa-spinner fa-spin text-gray-400';
                textClass = 'text-gray-500';
                break;
            case 'success':
                iconClass = 'fas fa-check-circle text-green-500';
                textClass = 'text-green-600';
                break;
            case 'error':
                iconClass = 'fas fa-times-circle text-red-500';
                textClass = 'text-red-600';
                break;
        }
        
        element.innerHTML = `
            <i class="${iconClass} mr-2"></i>
            <span class="${textClass}">${text}</span>
        `;
    }

    updateProgress(percent, text) {
        const progressBar = document.getElementById('progressBar');
        const progressText = document.getElementById('progressText');
        
        if (progressBar) {
            progressBar.style.width = `${percent}%`;
        }
        
        if (progressText) {
            progressText.textContent = text;
        }
    }

    showUploadProgress(show) {
        const uploadProgress = document.getElementById('uploadProgress');
        if (uploadProgress) {
            uploadProgress.classList.toggle('hidden', !show);
        }
    }

    initializeEventListeners() {
        // VN ID input handler
        document.getElementById('vnId').addEventListener('input', () => {
            this.updateGetRecommendationsButton();
        });

        // Get recommendations button handler
        document.getElementById('getRecommendations').addEventListener('click', () => {
            this.getRecommendations();
        });
    }

    updateGetRecommendationsButton() {
        const vnId = document.getElementById('vnId').value;
        const button = document.getElementById('getRecommendations');
        
        // Enable button if we have a VN ID and all data is loaded
        const allDataLoaded = this.dataLoaded.titles && this.dataLoaded.tags && this.dataLoaded.votes;
        const canGetRecommendations = vnId && this.recommender && allDataLoaded;
        button.disabled = !canGetRecommendations;
    }

    async getRecommendations() {
        const vnId = parseInt(document.getElementById('vnId').value);
        
        if (!vnId || vnId <= 0) {
            this.showError('Please enter a valid VN ID.');
            return;
        }

        if (!this.recommender) {
            this.showError('Recommendation engine not initialized.');
            return;
        }

        this.showLoading(true);
        this.hideError();
        this.hideResults();

        try {
            // Get the title first to verify the VN exists
            const title = this.recommender.get_title(vnId);
            if (!title || title.trim() === '') {
                this.showError(`Visual Novel with ID ${vnId} not found in the database.`);
                this.showLoading(false);
                return;
            }

            // Get recommendations
            const recommendations = this.recommender.get_recommendations(vnId);
            
            this.displayRecommendations(recommendations, title, vnId);
            this.showResults();
            
        } catch (error) {
            console.error('Error getting recommendations:', error);
            this.showError('Failed to get recommendations. Please try again.');
        } finally {
            this.showLoading(false);
        }
    }

    displayRecommendations(recommendations, sourceTitle, sourceId) {
        // Display combined recommendations
        this.displayRecommendationList(
            recommendations.combined_recommendations,
            'combinedRecommendations',
            sourceTitle,
            sourceId
        );

        // Display tag-based recommendations
        this.displayRecommendationList(
            recommendations.tag_recommendations,
            'tagRecommendations'
        );

        // Display user-based recommendations
        this.displayRecommendationList(
            recommendations.user_recommendations,
            'userRecommendations'
        );
    }

    displayRecommendationList(items, containerId, sourceTitle = null, sourceId = null) {
        const container = document.getElementById(containerId);
        container.innerHTML = '';

        if (sourceTitle && sourceId) {
            // Add source VN info for combined recommendations
            const sourceDiv = document.createElement('div');
            sourceDiv.className = 'bg-blue-50 border border-blue-200 rounded-lg p-4 mb-4';
            sourceDiv.innerHTML = `
                <div class="flex items-center">
                    <i class="fas fa-info-circle text-blue-500 mr-2"></i>
                    <span class="font-medium text-blue-800">Recommendations based on:</span>
                    <a href="https://vndb.org/v${sourceId}" target="_blank" 
                       class="ml-2 text-blue-600 hover:text-blue-800 font-semibold hover:underline">
                        ${sourceTitle} (ID: ${sourceId})
                    </a>
                </div>
            `;
            container.appendChild(sourceDiv);
        }

        if (items.length === 0) {
            container.innerHTML += `
                <div class="text-gray-500 text-center py-4">
                    <i class="fas fa-search mr-2"></i>
                    No recommendations found
                </div>
            `;
            return;
        }

        items.forEach((item, index) => {
            const itemDiv = document.createElement('div');
            itemDiv.className = 'recommendation-item bg-white border border-gray-200 rounded-lg p-4 hover:shadow-md transition-all duration-300';
            
            itemDiv.innerHTML = `
                <div class="flex items-center justify-between">
                    <div class="flex-1">
                        <span class="inline-flex items-center justify-center w-8 h-8 bg-blue-100 text-blue-800 text-sm font-medium rounded-full mr-3">
                            ${index + 1}
                        </span>
                        <span class="text-lg font-medium text-gray-900">${item.title}</span>
                        <span class="text-sm text-gray-500 ml-2">(ID: ${item.id})</span>
                    </div>
                    <a href="${item.url}" target="_blank" 
                       class="inline-flex items-center px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white text-sm font-medium rounded-lg transition duration-300">
                        <i class="fas fa-external-link-alt mr-2"></i>
                        View on VNDB
                    </a>
                </div>
            `;
            
            container.appendChild(itemDiv);
        });
    }

    showLoading(show) {
        const loading = document.getElementById('loading');
        loading.classList.toggle('hidden', !show);
    }

    showResults() {
        document.getElementById('results').classList.remove('hidden');
    }

    hideResults() {
        document.getElementById('results').classList.add('hidden');
    }

    showError(message) {
        const errorDiv = document.getElementById('error');
        const errorMessage = document.getElementById('errorMessage');
        errorMessage.textContent = message;
        errorDiv.classList.remove('hidden');
        
        // Auto-hide error after 5 seconds
        setTimeout(() => {
            this.hideError();
        }, 5000);
    }

    hideError() {
        document.getElementById('error').classList.add('hidden');
    }

    showSuccess(message) {
        // Create a temporary success message
        const successDiv = document.createElement('div');
        successDiv.className = 'bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4';
        successDiv.innerHTML = `
            <div class="flex">
                <div class="py-1">
                    <i class="fas fa-check-circle mr-2"></i>
                </div>
                <div>
                    <p class="font-bold">Success</p>
                    <p class="text-sm">${message}</p>
                </div>
            </div>
        `;
        
        // Insert after the upload section
        const uploadSection = document.querySelector('.container .bg-white');
        uploadSection.insertAdjacentElement('afterend', successDiv);
        
        // Remove after 3 seconds
        setTimeout(() => {
            successDiv.remove();
        }, 3000);
    }
}

// Initialize the app when the page loads
document.addEventListener('DOMContentLoaded', async () => {
    const app = new RecommendationApp();
    await app.initialize();
});
